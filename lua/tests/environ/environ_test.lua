return {
    groupName = "environ",

    cases = {
        {
            name = "Loads via require and creates the environ object",
            func = function()
                local ok, err = pcall( require, "environ" )
                if not ok then error( "require('environ') failed: " .. tostring( err ) ) end

                expect( environ ).to.exist()
            end
        },
        {
            name = "Reads a set environment variable as a string",
            func = function()
                -- PATH is set for every process the test container starts,
                -- and srcds inherits it.
                local path = environ.PATH

                expect( path ).to.beA( "string" )
                expect( #path ).to.beGreaterThan( 0 )
            end
        },
        {
            name = "Returns nil for an unset environment variable",
            func = function()
                expect( environ.GM_ENVIRON_DEFINITELY_NOT_SET ).to.beNil()
            end
        },
        {
            name = "Reads a variable the same way whether called with a dot or a colon",
            func = function()
                -- Both forms are supported deliberately; the key lands in a
                -- different argument slot depending on which one is used.
                expect( environ:get_csv( "PATH" ) ).to.deepEqual( environ.get_csv( "PATH" ) )
            end
        },
        {
            name = "Splits PATH into its component directories",
            func = function()
                local parts = environ.get_path()

                expect( parts ).to.beA( "table" )
                expect( #parts ).to.beGreaterThan( 0 )

                for _, part in ipairs( parts ) do
                    expect( part ).to.beA( "string" )
                    expect( #part ).to.beGreaterThan( 0 )

                    -- Entries come back individually split and trimmed, never
                    -- as one separator-joined blob.
                    expect( string.find( part, ":", 1, true ) ).to.beNil()
                    expect( part ).to.equal( string.Trim( part ) )
                end
            end
        },
        {
            name = "Splits a comma-separated variable into trimmed entries",
            func = function()
                local parts = environ.get_csv( "PATH" )

                expect( parts ).to.beA( "table" )
                expect( #parts ).to.beGreaterThan( 0 )

                for _, part in ipairs( parts ) do
                    expect( part ).to.beA( "string" )
                    expect( #part ).to.beGreaterThan( 0 )
                    expect( string.find( part, ",", 1, true ) ).to.beNil()
                    expect( part ).to.equal( string.Trim( part ) )
                end
            end
        },
        {
            name = "Returns an empty table when splitting an unset variable",
            func = function()
                local parts = environ.get_csv( "GM_ENVIRON_DEFINITELY_NOT_SET" )

                expect( parts ).to.beA( "table" )
                expect( #parts ).to.equal( 0 )
            end
        },
        {
            name = "Refuses to let Lua set an environment variable",
            func = function()
                expect( function()
                    environ.GM_ENVIRON_WRITE_TEST = "nope"
                end ).to.err()

                expect( environ.GM_ENVIRON_WRITE_TEST ).to.beNil()
            end
        },
        {
            name = "Refuses to let Lua overwrite an already-set variable",
            func = function()
                local before = environ.PATH

                expect( function()
                    environ.PATH = "/nowhere"
                end ).to.err()

                expect( environ.PATH ).to.equal( before )
            end
        },
        {
            name = "Resolves module functions ahead of same-named variables",
            func = function()
                -- Function names shadow the environment; these must come back
                -- as callables whether or not the host exports such a variable.
                for _, name in ipairs( { "get_path", "get_csv", "get_version", "get_build_info" } ) do
                    expect( environ[name] ).to.beA( "function" )
                end
            end
        },
        {
            name = "Splits PATH the same way whether called with a dot or a colon",
            func = function()
                expect( environ:get_path() ).to.deepEqual( environ.get_path() )
            end
        },
        {
            name = "Reports its own version as a non-empty string",
            func = function()
                local version = environ.get_version()
                expect( version ).to.beA( "string" )
                expect( #version ).to.beGreaterThan( 0 )
            end
        },
        {
            name = "Reports build info matching an official CI build of this target",
            func = function()
                local info = environ.get_build_info()
                expect( info ).to.exist()

                -- These tests only run against binaries built by ci.yml,
                -- so provenance should always resolve.
                expect( info.official ).to.equal( true )
                expect( info.commit ).to.beA( "string" )
                expect( #info.commit ).to.equal( 40 )
                expect( info.dirty ).to.equal( false )
                expect( info.version ).to.equal( environ.get_version() )
                expect( info.repository ).to.beA( "string" )
                expect( #info.repository ).to.beGreaterThan( 0 )
                expect( info.run_url ).to.beA( "string" )
                expect( #info.run_url ).to.beGreaterThan( 0 )

                -- CI always builds server modules on Linux.
                expect( info.realm ).to.equal( "sv" )
                expect( info.target:find( "linux", 1, true ) ).to.exist()
            end
        },
    }
}
