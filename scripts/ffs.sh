# For reasons we don't fully understand, regex101.com exports in an annoying manner:
< /tmp/deppat.json jq '[.[] | { "id":.[0] | .content, "name":.[1] | .content, "description":.[2] | .content, "price":.[3] | .content, }]' > content.json
