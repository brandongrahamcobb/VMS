require("telescope").setup({
	defaults = {
		file_ignore_patterns = {
			"AGENTS.md",
			"agent_docs",
			"HeavenClient",
			"game-data",
			"target",
			"integration%-harness",
			"node_modules",
			".git/",
			"README.md",
			"LICENSE",
			"GM_Book/",
		},
	},
})
