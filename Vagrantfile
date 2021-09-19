Vagrant.configure("2") do |config|
  config.vm.box = "ubuntu/focal64"
  config.vm.network "private_network", ip: "192.168.50.4"
  config.vm.provision "docker" do |d|
    d.build_image "/vagrant/rust-image",
      args: "-t rust:nightly"
    d.build_image "/vagrant/users-microservice",
      args: "-t users-microservice:latest"
    d.run "users-microservice:latest",
      args: "-p 8080:8080"
  end
end

