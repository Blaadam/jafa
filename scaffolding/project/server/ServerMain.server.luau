--!nocheck
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local ServerScriptService = game:GetService("ServerScriptService")
local ServerStorage = game:GetService("ServerStorage")
local Workspace = game:GetService("Workspace")

local Loader = require(ReplicatedStorage.Shared.Packages.Loader)

local Logger = require(ReplicatedStorage.Shared.Modules.Logger)
local Log = Logger.new()

local PRODUCTION_PLACE_IDS = { -1 }

function HideDebuggingUtils(list: Instance)
	local adornments = list:QueryDescendants("ConeHandleAdornment, SphereHandleAdornment")
	for _, v in adornments do
		v:Destroy()
	end

	local interactions = list:QueryDescendants("BasePart [Transparency = 0] > .Interaction")
	for _, v in interactions do
		v.Transparency = 1
	end
end

local Success, Response = pcall(function()
	if not table.find(PRODUCTION_PLACE_IDS, game.PlaceId) then
		Workspace:SetAttribute("IsDevEnvironment", true)
	end

	Log:AtTrace():Log("Requiring Server Services")
	local LoadedServices = Loader.LoadDescendants(ServerScriptService.Services, Loader.MatchesName("Service"))

	Log:AtTrace():Log("Starting Server Services")
	Loader.SpawnAll(LoadedServices, "OnStart")

	Log:AtTrace():Log(`All Server Services and Facilitators have been started. Handling Debug Elements`)

	HideDebuggingUtils(Workspace)
	HideDebuggingUtils(ReplicatedStorage)
	HideDebuggingUtils(ServerStorage)

	Log:AtTrace():Log(`Debug Elements have been handled. Server is configured.`)
end)

if not Success then
	Log:AtFatal():Log(Response)

	local function kickPlayer(player: Player)
		player:Kick("An error occured whilst starting the server. Error: \n" .. Response)
	end

	for _, player: Player in pairs(Players:GetPlayers()) do
		kickPlayer(player)
	end

	Players.PlayerAdded:Connect(kickPlayer)
end
