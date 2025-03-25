package main

import "github.com/meta-quick/godrl/pkg/odrl"

func main() {
	jsonld := `
{
	"@context": [
		"https://www.w3.org/ns/odrl.jsonld",
		{
			"title": "https://datasafe.io/ds/1.1/title",
			"creator": "https://datasafe.io/ds/1.1/creator",
			"dateCreated": "https://datasafe.io/ds/1.1/dateCreated"
		}
	],
	"type": "Agreement",
	"uid": "https://datasate.ids/aggreement/00001",
	"assigner": {
		"uid": "https://datasate.ids/users/gaosg",
		"type": "Party",
		"assignerOf": "https://datasate.ids/dataset/00001",
		"refinement": {
            "dataType": "dateTime",
            "unit": "m",
            "leftOperand": "dateTime",
            "operator": "lt",
            "rightOperand": "2025-12-31"
		}
	},
	"assignee": {
		"uid": "https://datasate.ids/usercollection/liumazi",
		"type": "PartyCollection",
		"source": "https://datasate.ids/usercollection/liumazi",
		"refinement": {
            "dataType": "dateTime",
            "unit": "m",
            "leftOperand": "dateTime",
            "operator": "lt",
            "rightOperand": "2025-12-31"
        }
	},
    "target": "https://datasate.ids/llm/dataset/0001",
	"title": "Policy 1",
	"conflict": "Perm",
	"inheritFrom": [],
	"profile": "https://datasate.ids/profiles/0001",
	"permission": [{
			"action": "use",
			"assignee": "https://datasate.ids/usercollection/liumazi",
			"target": ["https://datasate.ids/llm/dataset/0001","https://datasate.ids/llm/dataset/0001"],
			"constraint": {
				"dataType": "dateTime",
				"unit": "m",
				"leftOperand": "dateTime",
				"operator": "lt",
				"rightOperand": "2025-12-31"
			}
		},
		{
			"action": "use",
			"constraint": {
				"type": "LogicalConstraint",
				"uid": "https://datasate.ids/users/gaosg",
				"operator": "and",
				"constraint": [{
				        "unit": "cm",
				        "dataType": "dateTime",
						"leftOperand": "dateTime",
						"operator": "gt",
						"rightOperand": "2025-12-31"
					},
					{
					    "dataType": "dateTime",
						"leftOperand": "dateTime",
						"operator": "lt",
						"rightOperand": "2025-12-31"
					}
				]
			}
		}
	]
}
    `

	engine := odrl.NewEngine(false, jsonld)
	defer engine.Close()

	/**
	  sftp user -- odrl user mapping (user req exec contract)
	*/

	result := engine.Eval("http://www.w3.org/ns/odrl/2/use",
		"https://datasate.ids/llm/dataset/0001",
		"https://datasate.ids/users/gaosg",
		"https://datasate.ids/usercollection/liumazi")
	println(result)

	engine.UpdateWorld("https://datasate.ids/abc", "123")
	engine.UpdateWorld("https://datasate.ids/def", "456")

	a := engine.FetchWorld("https://datasate.ids/abc")
	println(a)

	a = engine.FetchWorld("https://datasate.ids/def")
	println(a)
}
