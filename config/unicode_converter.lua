print(vim.inspect(vim.api.nvim_list_runtime_paths()))
vim.opt.runtimepath:append(',~/.config/nvim/lua')

vim.api.nvim_set_keymap('v', '<leader>u', ':lua require("nvim_unicode_converter").convert_unicode()<CR>', { noremap = true, silent
= true })