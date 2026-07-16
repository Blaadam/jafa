local roblox = require("@lune/roblox")
local net = require("@lune/net")
local fs = require("@lune/fs")
local serde = require("@lune/serde")
local process = require("@lune/process")

local cookie: string? = roblox.getAuthCookie()
assert(cookie ~= nil, "Failed to get roblox auth cookie")

type AssetNode = string | { [string]: AssetNode }

type PlaceTree = {
	game_name: string?, -- Optional, helps with verbosity when printing errors.
	place_id: number | string,
	universe_id: number | string,
	assets: { [string]: AssetNode },
}

type JafaFile = {
	jafa_workspace: string,
	places: { PlaceTree },
}

if #process.args == 0 then
	print("Usage: lune run greet <name>")
	print("Example: lune run greet Alice")
end

function fetchPlace(placeId: number | string): roblox.DataModel?
	if tostring(placeId) == "" or tostring(placeId) == "0" then
		return
	end

	local response = net.request({
		url = `https://assetdelivery.roblox.com/v2/assetId/{placeId}`,
		headers = {
			Cookie = cookie,
		},
	})

	local responseTable: any = serde.decode("json", response.body)

	local place_response: net.FetchResponse = net.request({
		url = responseTable.locations[1].location,
	})

	return roblox.deserializePlace(place_response.body)
end

function retrieveJafaProject(projectName: string): (boolean, JafaFile | string, string?)
	local projectDir: string = `./projects/{projectName}`
	local isFile: boolean = fs.isFile(`{projectDir}/.jafa`)
	if not isFile then
		return false, `Project "{projectName}" does not exist or is not a valid Jafa project.`, nil
	end

	local jafaFileContent: string
	local jafaData: JafaFile

	local success: boolean, errorMessage: string? = pcall(function()
		jafaFileContent = fs.readFile(`{projectDir}/.jafa`)
		jafaData = serde.decode("json", jafaFileContent)
	end)

	if not success then
		return false, `Failed to parse .jafa file for project "{projectName}": {errorMessage}`
	end

	return true, jafaData, projectDir
end

function GetRobloxPath(current: roblox.Instance, dir: string)
	local segments: { string } = dir:split("/")
	for _, v: string in pairs(segments) do
		if v ~= "" and v ~= "." then
			current = current[v]
		end
	end

	return current
end

function TreeMapping(node: AssetNode, path: string, output: { [string]: string })
	if typeof(node) == "string" then
		output[path] = node
	elseif typeof(node) == "table" then
		for key: string, child: AssetNode in pairs(node) do
			TreeMapping(child, path .. "/" .. key, output)
		end
	end
end

function AssetLoop(
	place: roblox.DataModel,
	placeId: string | number,
	placeName: string,
	robloxPath: string,
	assetPath: string
)
	local assetInstance = GetRobloxPath(place, robloxPath)
	if not assetInstance then
		warn(`Asset at path "{robloxPath}" not found in place "{placeId}" ({placeName}).`)
		return
	end

	local outputFilePath: string = assetPath
	local outputDir: string? = outputFilePath:match("^(.*)/[^/]+$")
	if outputDir then
		local builtPath: string = ""
		for _, segment: string in outputDir:split("/") do
			builtPath = builtPath == "" and segment or (builtPath .. "/" .. segment)
			if builtPath ~= "" then
				fs.writeDir(builtPath)
			end
		end
	end

	local serializedData: string = roblox.serializeModel({ assetInstance })
	fs.writeFile(outputFilePath, serializedData)
	print(`Saved asset from "{robloxPath}" to "{outputFilePath}".`)
end

function PlaceLoop(jafaPlace: PlaceTree, projectDir: string)
	local placeId: number | string = jafaPlace.place_id
	local placeName: string = jafaPlace.game_name or "Unknown"

	print(`Processing place "{placeId}" ({placeName})...`)

	local place: roblox.DataModel? = fetchPlace(placeId)
	if not place then
		warn(`Failed to fetch place "{placeId}" ({placeName}). Skipping.`)
		return
	end

	local treeItems: { [string]: string } = {}
	TreeMapping(jafaPlace.assets, ".", treeItems)

	for robloxPath: string, assetPath: string in treeItems do
		robloxPath = robloxPath:gsub("^%./", "") -- Remove leading "./" if present
		assetPath = assetPath:gsub("project://", projectDir .. "/") -- Replace project:// with the actual project directory

		local success: boolean, errorMsg: string? = pcall(function()
			AssetLoop(place, placeId, placeName, robloxPath, assetPath)
		end)
		if not success then
			warn(`Error processing asset "{robloxPath}" in place "{placeId}" ({placeName}): {errorMsg}`)
		end
	end

	print(`Found assets to process for place "{placeId}" ({placeName}):`)
	print(treeItems)
end

function SaveProjectAssets(project: string): boolean
	local success: boolean, jafaData: JafaFile | string, projectDir: string? = retrieveJafaProject(project)
	if (not success) or typeof(jafaData) == "string" then
		error(`Failed to retrieve Jafa project "{project}": {jafaData}`)
		return false
	end

	if not projectDir then
		error(`Project directory for "{project}" is nil.`)
		return false
	end

	for _, place in jafaData.places do
		local success_place: boolean, errorMessage: string? = pcall(function()
			PlaceLoop(place, projectDir)
		end)

		if not success_place then
			warn(`Error processing place "{place.place_id}" ({place.game_name}): {errorMessage}`)
		end
	end

	return true
end

for _, project in process.args do
	local success: boolean, errorMessage: string? = pcall(function()
		SaveProjectAssets(project)
	end)

	if not success then
		warn(`Error processing project "{project}": {errorMessage}`)
	end
end
