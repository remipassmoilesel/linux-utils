
custom_namespace = {
    address = nil
}

function conky_public_address()
    if (custom_namespace['address'] == nil) then
        local curl_command = io.popen("curl ipinfo.io/ip")
        local address = curl_command:read()
        custom_namespace['address'] = address
    end
    return custom_namespace['address']
end
