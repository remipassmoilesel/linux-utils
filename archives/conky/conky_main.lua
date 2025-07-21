
-- TODO: list all .lua files from dir
function importAllFiles()
    local homeDirectory = os.getenv("HOME")
    local filesToImport = {
        'conky_helpers.lua',
        'conky_draw_templates.lua',
        'conky_draw_config.lua',
        'conky_draw.lua',
    }

    for index, fileName in ipairs(filesToImport) do
        local fullPath = homeDirectory .. '/.conky/remipassmoilesel/' .. fileName
        dofile(fullPath)
    end
end

importAllFiles()
