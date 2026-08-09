last_job = 0

function init(plugin)
	plugin:newCommand {
		id = "test_command",
		title = "Test Extension",
		group = "file_import",
		onclick = function()
			loadlib(plugin)

			app.alert(libtest.hello())

			last_job = libtest.http_get("https://sefa.moe")
			app.alert("Job Result: " .. libtest.check_job(last_job))

			app.alert("Job: " .. last_job)
		end,
	}

	plugin:newCommand {
		id = "test_check",
		title = "Check Test Extension Job",
		group = "file_import",
		onclick = function()
			loadlib(plugin)

			local result = libtest.check_job(last_job)

			app.alert("Job Result: " .. result)
		end,
	}
end

function exit(plugin)
end

function loadlib(plugin)
	if not libtest then
		local name = app.os.name == "Windows" and "test.dll" or app.os.name == "macOS" and "libtest.dylib" or "libtest.so"
		local loader, err = package.loadlib(app.fs.joinPath(plugin.path, name), "luaopen_libtest")

		if not loader then
			app.alert("Error loading test library: " .. err)
		else
			libtest = loader()
		end
	end
end
