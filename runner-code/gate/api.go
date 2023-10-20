package main

import (
	"fmt"
	"net/http"
	"os"
	"github.com/gin-gonic/gin"
	"gopkg.in/yaml.v3"
)

type Config struct {
	Config struct {
		Lite struct {
			Enabled bool `yaml:"enabled"`
			Routes  []Route
		} `yaml:"lite"`
	} `yaml:"config"`
}

type Route struct {
	Host    string `yaml:"host"`
	Backend string `yaml:"backend"`
}

var m = make(map[string]string)

func update_config() {
	config := Config{}
	config.Config.Lite.Enabled = true;

	var routes []Route
	for host, backend := range m {
		routes = append(routes, Route{Host: host, Backend: backend})
	}
	config.Config.Lite.Routes = routes

	yamlData, err := yaml.Marshal(&config)
	if err != nil {
		fmt.Println(err)
		return
	}

	err = os.WriteFile("./config/config.yml", yamlData, 0644)
	if err != nil {
		fmt.Println(err)
		return
	}
}

func createHost(c *gin.Context) {
	host := c.PostForm("host")
	backend := c.PostForm("backend")
	m[host] = backend
	update_config()
	c.JSON(http.StatusOK, gin.H{"message": "Host created successfully"})
}

func deleteHost(c *gin.Context) {
	host := c.Param("host")
	delete(m, host)
	update_config()
	c.JSON(http.StatusOK, gin.H{"message": "Host deleted successfully"})
}

func main() {
	r := gin.Default()

	r.POST("/createhost", createHost)
	r.DELETE("/deletehost/:host", deleteHost)

	if err := r.Run(":3000"); err != nil {
		fmt.Println(err)
	}
}
