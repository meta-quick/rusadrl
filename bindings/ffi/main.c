/*
 * Copyright 2024 meduo <gao.brian@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include <stdio.h>
#include <stdlib.h>
#include "include/odrl.h"

int main() {
    enable_verbose(1);

const char* json_data = "{\n"
    "    \"@context\": [\n"
    "        \"https://www.w3.org/ns/odrl.jsonld\",\n"
    "        {\n"
    "            \"title\": \"https://datasafe.io/ds/1.1/title\",\n"
    "            \"creator\": \"https://datasafe.io/ds/1.1/creator\",\n"
    "            \"dateCreated\": \"https://datasafe.io/ds/1.1/dateCreated\"\n"
    "        }\n"
    "    ],\n"
    "    \"type\": \"Agreement\",\n"
    "    \"uid\": \"http://abc.tds/policy/demo/1\",\n"
    "    \"assigner\": {\n"
    "        \"uid\": \"https://aa/bb/gaosg\",\n"
    "        \"type\": \"Party\",\n"
    "        \"assignerOf\": \"http://aaa/a\",\n"
    "        \"refinement\": {\n"
    "            \"dataType\": \"integer\",\n"
    "            \"unit\": \"m\",\n"
    "            \"leftOperand\": \"dateTime\",\n"
    "            \"operator\": \"lt\",\n"
    "            \"rightOperand\": \"abc\"\n"
    "        }\n"
    "    },\n"
    "    \"assignee\": {\n"
    "        \"uid\": \"https://aa/gaosg\",\n"
    "        \"type\": \"PartyCollection\",\n"
    "        \"source\": \"https://aa.com/aaa\",\n"
    "        \"refinement\": {\n"
    "            \"dataType\": \"integer\",\n"
    "            \"unit\": \"m\",\n"
    "            \"leftOperand\": \"dateTime\",\n"
    "            \"operator\": \"lt\",\n"
    "            \"rightOperand\": \"abc\"\n"
    "        }\n"
    "    },\n"
    "    \"target\": \"http://ab/a\",\n"
    "    \"title\": \"Policy 1\",\n"
    "    \"conflict\": \"Perm\",\n"
    "    \"inheritFrom\": [\"http://a.com/abc\", \"http://a.com/abc\"],\n"
    "    \"profile\": \"http://a.com/abc\",\n"
    "    \"permission\": [\n"
    "        {\n"
    "            \"action\": \"use\",\n"
    "            \"assignee\": \"http://abc/liumazi\",\n"
    "            \"constraint\": {\n"
    "                \"dataType\": \"integer\",\n"
    "                \"unit\": \"m\",\n"
    "                \"leftOperand\": \"dateTime\",\n"
    "                \"operator\": \"lt\",\n"
    "                \"rightOperand\": \"abc\"\n"
    "            }\n"
    "        },\n"
    "        {\n"
    "            \"action\": \"use\",\n"
    "            \"constraint\": {\n"
    "                \"type\": \"LogicalConstraint\",\n"
    "                \"uid\": \"http://example.com/constraint/1\",\n"
    "                \"operator\": \"and\",\n"
    "                \"constraint\": [\n"
    "                    {\n"
    "                        \"leftOperand\": \"dateTime\",\n"
    "                        \"operator\": \"gt\",\n"
    "                        \"rightOperandReference\": \"http://a/a\"\n"
    "                    },\n"
    "                    {\n"
    "                        \"leftOperand\": \"dateTime\",\n"
    "                        \"operator\": \"lt\",\n"
    "                        \"rightOperand\": \"2025-12-31\"\n"
    "                    }\n"
    "                ]\n"
    "            }\n"
    "        }\n"
    "    ]\n"
    "}";


    create_odrl_world(json_data);

    return 0;
}