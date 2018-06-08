local homeDirectory = os.getenv("HOME")
local filesToImport = {
    'conky_draw_config.lua',
    'conky_helpers.lua',
    'conky_draw.lua',
}

for index, fileName in ipairs(filesToImport) do
    local fullPath = homeDirectory .. '/.conky/remipassmoilesel/' .. fileName
    dofile(fullPath)
end
