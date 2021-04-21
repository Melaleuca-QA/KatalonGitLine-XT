import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response1 = WS.sendRequest(findTestObject('LINE - XT/1.1 XT - Get Access Token', [('Tokenkey') : GlobalVariable.G2_TokenKey]))
println('.. response1 is: .. ' + response1)

def slurper1 = new groovy.json.JsonSlurper()
def result1 = slurper1.parseText(response1.getResponseBodyContent())
def value1 = result1.access_token

GlobalVariable.G_TokenKey = value1
println('.. G_TokenKey is: .. ' + GlobalVariable.G_TokenKey)

response2 = WS.sendRequest(findTestObject('LINE - XT/1.2 Search Customer', [('Melaleucaid') : GlobalVariable.G_MelaleucaId, ('PhoneNumber') : GlobalVariable.G_PhoneNumber
            , ('Tokenkey') : GlobalVariable.G_TokenKey, ('CustomerID') : GlobalVariable.G_CustomerID]))
println('.. response2 is: .. ' + response2)

def slurper2 = new groovy.json.JsonSlurper()
def result2 = slurper2.parseText(response2.getResponseBodyContent())
def value2 = result2.CustomerID

GlobalVariable.G_CustomerID = value2
println('.. G_CustomerId is: .. ' + value2)

response3 = WS.sendRequest(findTestObject('LINE - XT/2.1 XT - Get Password Access Token', [('username') : GlobalVariable.G2_username
            , ('password') : GlobalVariable.G2_password, ('TokenKey') : GlobalVariable.G2_TokenKey]))
println('.. response3 is: .. ' + response3)

def slurper3 = new groovy.json.JsonSlurper()
def result3 = slurper3.parseText(response3.getResponseBodyContent())
def value3 = result3.access_token

GlobalVariable.G2_TokenKey = value3
println('.. G2_TokenKey is: .. ' + GlobalVariable.G2_TokenKey)

response4 = WS.sendRequest(findTestObject('LINE - XT/3.2 Delete Contacts', [('CustomerId') : GlobalVariable.G_CustomerID, ('TokenKey') : GlobalVariable.G2_TokenKey
            , ('Contact') : GlobalVariable.G3_Contact, ('SpouseContact') : GlobalVariable.G3_SpouseContact]))
println('.. response4 is: .. ' + response4)

