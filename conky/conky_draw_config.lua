
function conky_get_cpu_graph_configurations()

    local graphGap = 13

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
    --
    config.conky_value = 'memperc'
    config.critical_threshold = 70
    --
    config.from = {x = 150, y = y}
    config.to = {x = 325, y = y}
    --
    config.bar_thickness = 15
    config.bar_thickness_critical = 15
    --
    config.background_thickness = 15
    config.background_thickness_critical = 15
    return config
end

local allGraphs = conky_get_cpu_graph_configurations()
table.insert(allGraphs, conky_get_mem_graph(713))
table.insert(allGraphs, conky_get_temperature_graph(915))
table.insert(allGraphs, conky_get_dir_graph('/home', 1005))
table.insert(allGraphs, conky_get_dir_graph('/', 950))

elements = allGraphs
