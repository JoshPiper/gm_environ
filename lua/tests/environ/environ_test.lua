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
    }
}
