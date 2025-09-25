#!/bin/bash
set -e

# If the first argument starts with a dash or is a subcommand, prepend git-iris command
if [ "${1:0:1}" = "-" ] || [ "$1" = "gen" ] || [ "$1" = "review" ] || [ "$1" = "changelog" ] || [ "$1" = "release-notes" ] || [ "$1" = "config" ] || [ "$1" = "list-presets" ]; then
    set -- git-iris "$@"
fi

# Setup git config if environment variables are provided
if [ -n "$GIT_USER_NAME" ]; then
    git config --global user.name "$GIT_USER_NAME"
fi

if [ -n "$GIT_USER_EMAIL" ]; then
    git config --global user.email "$GIT_USER_EMAIL"
fi

# Create Git-Iris config directory
mkdir -p ~/.config/git-iris

# Initialize config settings
CONFIG_PARAMS=""

# Process provider config
if [ -n "$GITIRIS_PROVIDER" ]; then
    CONFIG_PARAMS="$CONFIG_PARAMS --provider $GITIRIS_PROVIDER"

    # API key for the provider (if provided)
    if [ -n "$GITIRIS_API_KEY" ]; then
        CONFIG_PARAMS="$CONFIG_PARAMS --api-key $GITIRIS_API_KEY"
    fi

    # Model for the provider (if provided)
    if [ -n "$GITIRIS_MODEL" ]; then
        CONFIG_PARAMS="$CONFIG_PARAMS --model $GITIRIS_MODEL"
    fi

    # Token limit for the provider (if provided)
    if [ -n "$GITIRIS_TOKEN_LIMIT" ]; then
        CONFIG_PARAMS="$CONFIG_PARAMS --token-limit $GITIRIS_TOKEN_LIMIT"
    fi
fi

# Default provider setting
if [ -n "$GITIRIS_DEFAULT_PROVIDER" ]; then
    CONFIG_PARAMS="$CONFIG_PARAMS --default-provider $GITIRIS_DEFAULT_PROVIDER"
fi

# Custom instructions
if [ -n "$GITIRIS_INSTRUCTIONS" ]; then
    CONFIG_PARAMS="$CONFIG_PARAMS --instructions \"$GITIRIS_INSTRUCTIONS\""
fi

# Preset
if [ -n "$GITIRIS_PRESET" ]; then
    CONFIG_PARAMS="$CONFIG_PARAMS --preset $GITIRIS_PRESET"
fi

# Gitmoji setting
if [ -n "$GITIRIS_GITMOJI" ]; then
    CONFIG_PARAMS="$CONFIG_PARAMS --gitmoji $GITIRIS_GITMOJI"
fi

# Additional parameters (comma-separated key=value pairs)
if [ -n "$GITIRIS_PARAMS" ]; then
    # Split the comma-separated list and add each parameter
    IFS=',' read -ra PARAM_ARRAY <<<"$GITIRIS_PARAMS"
    for param in "${PARAM_ARRAY[@]}"; do
        CONFIG_PARAMS="$CONFIG_PARAMS --param $param"
    done
fi

# Apply configuration if parameters were provided
if [ -n "$CONFIG_PARAMS" ]; then
    # Only configure if it's not already done or if forced
    if [ ! -f ~/.config/git-iris/config.toml ] || [ "$GITIRIS_FORCE_CONFIG" = "true" ]; then
        eval "git-iris config $CONFIG_PARAMS"
    fi
fi

# If no command is provided, print help
if [ "$1" = "git-iris" ] && [ $# -eq 1 ]; then
    exec "$@" --help
else
    exec "$@"
fi
