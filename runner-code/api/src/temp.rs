create_server(options, file_id) {
	curl data from SeaweedFS using datapath
	run itzg/docker image (binding in the downloaded folder)
	write the server into gate
	return the uuid of the server
}

When the user makes a request to the backend, the database 
is queried for the UUID -> file_id relationship


fn download_data(file_id) -> String {
	// Get the server data from SeaweedFS
	Command::new("curl")
	.arg(format!("127.0.0.1:8080/{}", file_id))
	.arg("-o")
	.arg(format!("{}.zst", file_id))

	// Unzip the server data using zstd
	
	Unzip

	/minecraft/file_id
}