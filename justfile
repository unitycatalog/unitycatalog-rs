# generate delta-sharing types from proto files
generate:
    cd proto && buf dep update
    @buf generate proto
