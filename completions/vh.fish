# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_vh_global_optspecs
	string join \n v/verbose h/help V/version
end

function __fish_vh_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_vh_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_vh_using_subcommand
	set -l cmd (__fish_vh_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c vh -n "__fish_vh_needs_command" -s v -l verbose -d 'Enable verbose logging output'
complete -c vh -n "__fish_vh_needs_command" -s h -l help -d 'Print help'
complete -c vh -n "__fish_vh_needs_command" -s V -l version -d 'Print version'
complete -c vh -n "__fish_vh_needs_command" -f -a "create" -d 'Create or update a local development domain'
complete -c vh -n "__fish_vh_needs_command" -f -a "list" -d 'List all managed domains'
complete -c vh -n "__fish_vh_needs_command" -f -a "describe" -d 'View detailed information and integration paths for a domain'
complete -c vh -n "__fish_vh_needs_command" -f -a "remove" -d 'Remove a managed domain'
complete -c vh -n "__fish_vh_needs_command" -f -a "allow-ext" -d 'Allow a custom domain extension (TLD)'
complete -c vh -n "__fish_vh_needs_command" -f -a "remove-ext" -d 'Remove a custom domain extension (TLD) from the allowed list'
complete -c vh -n "__fish_vh_needs_command" -f -a "completions" -d 'Generate shell completion scripts'
complete -c vh -n "__fish_vh_needs_command" -f -a "ca" -d 'Show path to the Root CA certificate and installation instructions'
complete -c vh -n "__fish_vh_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c vh -n "__fish_vh_using_subcommand create" -s i -l ip -d 'IP address to point the domain to' -r
complete -c vh -n "__fish_vh_using_subcommand create" -s v -l verbose -d 'Enable verbose logging output'
complete -c vh -n "__fish_vh_using_subcommand create" -s h -l help -d 'Print help'
complete -c vh -n "__fish_vh_using_subcommand list" -s v -l verbose -d 'Enable verbose logging output'
complete -c vh -n "__fish_vh_using_subcommand list" -s h -l help -d 'Print help'
complete -c vh -n "__fish_vh_using_subcommand describe" -s v -l verbose -d 'Enable verbose logging output'
complete -c vh -n "__fish_vh_using_subcommand describe" -s h -l help -d 'Print help'
complete -c vh -n "__fish_vh_using_subcommand remove" -s v -l verbose -d 'Enable verbose logging output'
complete -c vh -n "__fish_vh_using_subcommand remove" -s h -l help -d 'Print help'
complete -c vh -n "__fish_vh_using_subcommand allow-ext" -s v -l verbose -d 'Enable verbose logging output'
complete -c vh -n "__fish_vh_using_subcommand allow-ext" -s h -l help -d 'Print help'
complete -c vh -n "__fish_vh_using_subcommand remove-ext" -s v -l verbose -d 'Enable verbose logging output'
complete -c vh -n "__fish_vh_using_subcommand remove-ext" -s h -l help -d 'Print help'
complete -c vh -n "__fish_vh_using_subcommand completions" -s v -l verbose -d 'Enable verbose logging output'
complete -c vh -n "__fish_vh_using_subcommand completions" -s h -l help -d 'Print help'
complete -c vh -n "__fish_vh_using_subcommand ca" -s v -l verbose -d 'Enable verbose logging output'
complete -c vh -n "__fish_vh_using_subcommand ca" -s h -l help -d 'Print help'
complete -c vh -n "__fish_vh_using_subcommand help; and not __fish_seen_subcommand_from create list describe remove allow-ext remove-ext completions ca help" -f -a "create" -d 'Create or update a local development domain'
complete -c vh -n "__fish_vh_using_subcommand help; and not __fish_seen_subcommand_from create list describe remove allow-ext remove-ext completions ca help" -f -a "list" -d 'List all managed domains'
complete -c vh -n "__fish_vh_using_subcommand help; and not __fish_seen_subcommand_from create list describe remove allow-ext remove-ext completions ca help" -f -a "describe" -d 'View detailed information and integration paths for a domain'
complete -c vh -n "__fish_vh_using_subcommand help; and not __fish_seen_subcommand_from create list describe remove allow-ext remove-ext completions ca help" -f -a "remove" -d 'Remove a managed domain'
complete -c vh -n "__fish_vh_using_subcommand help; and not __fish_seen_subcommand_from create list describe remove allow-ext remove-ext completions ca help" -f -a "allow-ext" -d 'Allow a custom domain extension (TLD)'
complete -c vh -n "__fish_vh_using_subcommand help; and not __fish_seen_subcommand_from create list describe remove allow-ext remove-ext completions ca help" -f -a "remove-ext" -d 'Remove a custom domain extension (TLD) from the allowed list'
complete -c vh -n "__fish_vh_using_subcommand help; and not __fish_seen_subcommand_from create list describe remove allow-ext remove-ext completions ca help" -f -a "completions" -d 'Generate shell completion scripts'
complete -c vh -n "__fish_vh_using_subcommand help; and not __fish_seen_subcommand_from create list describe remove allow-ext remove-ext completions ca help" -f -a "ca" -d 'Show path to the Root CA certificate and installation instructions'
complete -c vh -n "__fish_vh_using_subcommand help; and not __fish_seen_subcommand_from create list describe remove allow-ext remove-ext completions ca help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
