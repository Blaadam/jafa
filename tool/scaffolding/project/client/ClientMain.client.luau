--!nocheck
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local Loader = require(ReplicatedStorage.Shared.Packages.Loader)
local Logger = require(ReplicatedStorage.Shared.Modules.Logger)
local Log = Logger.new()

if not game:IsLoaded() then
	game.Loaded:Wait()
end

local Success, Response = pcall(function()
	Log:AtInfo():Log("Requiring Client Services")
	local LoadedServices = Loader.LoadDescendants(ReplicatedStorage.Client.Services)

	Log:AtInfo():Log("Starting Client Services")
	Loader.SpawnAll(LoadedServices, "OnInit")

	Loader.SpawnAll(LoadedServices, "OnStart")

	Log:AtInfo():Log("Requiring Client Controllers")
	local LoadedControllers = Loader.LoadDescendants(ReplicatedStorage.Client.Controllers)

	Log:AtInfo():Log("Starting Client Controllers")
	Loader.SpawnAll(LoadedControllers, "OnStart")
end)

if not Success then
	Log:AtFatal():Log(Response)
end
