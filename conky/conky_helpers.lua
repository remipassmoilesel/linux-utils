
network_cache = {
    public_address = nil,
    private_addresses = nil,
}

function conky_command(command)
    local commandOutput = io.popen(command)
    return commandOutput:read('*a')
end

function conky_public_address()
    if (network_cache.public_address == nil) then
        local public_address = conky_command("curl ipinfo.io/ip"):sub(0, -2)
        network_cache.public_address = public_address
    end
    return network_cache.public_address
end

function conky_private_addresses()
    if (network_cache.private_addresses == nil) then
        local command = "ip a | grep -Eo 'inet (addr:)?([0-9]*\\.){3}[0-9]*' | grep -Eo '([0-9]*\\.){3}[0-9]*' | grep -v '127.0.0.1'"
        local addressesStr = conky_command(command):sub(0, -2)
        local addresses = string.gmatch(addressesStr, "[%d%p]+")
        local filteredAddresses = {}
        for addr in addresses do
            if addr:sub(0, 3) ~= "172" then
                table.insert(filteredAddresses, addr)
            end
        end
        network_cache.private_addresses = table.concat(filteredAddresses, ', ')
    end
    return network_cache.private_addresses
end
