
function conky_get_cpu_graph_configurations()

    local graphGap = 14

    local cpuGraphsConfigurations = {}
    local cpuNumber = tonumber(conky_command('grep processor /proc/cpuinfo | wc -l '))

    for cpuIndex = 0, cpuNumber do
        local config = conky_get_cpu_graph_template()
        config.conky_value = 'cpu cpu' .. cpuIndex
        config.radius = config.radius + cpuIndex * graphGap
        table.insert(cpuGraphsConfigurations, config)
    end

    return cpuGraphsConfigurations
end

function conky_get_temperature_graph(y)
    local config = conky_get_bar_template()
    --
    config.conky_value = 'acpitemp'
    config.critical_threshold = 65
    config.max_value = 90
    --
    config.from = {x = 200, y = y}
    config.to = {x = 325, y = y}
    --
    return config
end

function conky_get_dir_graph(path, y)
    local config = conky_get_bar_template()
    --
    config.conky_value = 'fs_used_perc ' .. path
    config.critical_threshold = 85
    config.max_value = 100
    --
    config.from = {x = 200, y = y}
    config.to = {x = 325, y = y}
    --
    return config
end

function conky_get_mem_graph(y)
    local config = conky_get_bar_template()
    local thickness = 12
    --
    config.conky_value = 'memperc'
    config.critical_threshold = 70
    --
    config.from = {x = 170, y = y}
    config.to = {x = 325, y = y}
    --
    config.bar_thickness = thickness
    config.bar_thickness_critical = thickness
    --
    config.background_thickness = thickness
    config.background_thickness_critical = thickness
    return config
end

local offset = 719
local allGraphs = conky_get_cpu_graph_configurations()
table.insert(allGraphs, conky_get_mem_graph(offset))
table.insert(allGraphs, conky_get_temperature_graph(offset + 200))
table.insert(allGraphs, conky_get_dir_graph('/', offset + 235))
table.insert(allGraphs, conky_get_dir_graph('/home', offset + 290))

elements = allGraphs
