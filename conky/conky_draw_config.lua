
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

function conky_get_temperature_graph()
    local config = conky_get_thin_bar_template()
    config.kind = 'bar_graph'
    config.conky_value = 'acpitemp'
    config.critical_threshold = 70
    config.max_value = 90
    config.from = {x = 220, y = 915}
    config.to = {x = 320, y = 915}
    return config
end

local allGraphs = conky_get_cpu_graph_configurations()
local tempGraph = conky_get_temperature_graph()

table.insert(allGraphs, tempGraph)

elements = allGraphs
