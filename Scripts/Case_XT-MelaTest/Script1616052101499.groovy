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

response1 = WS.sendRequest(findTestObject('LINE - XT/1.1 XT - Get Access Token', [('Tokenkey') : GlobalVariable.G_TokenKey]))
println('.. response1 is: .. ' + response1)

def slurper1 = new groovy.json.JsonSlurper()
def result1 = slurper1.parseText(response1.getResponseBodyContent())
def value1 = result1.access_token

GlobalVariable.G_TokenKey = value1

def value2_0 = GlobalVariable.G_TokenKey
def value2_1 = GlobalVariable.G_MelaleucaId
def value2_2 = GlobalVariable.G_PhoneNumber

println('.. G_TokenKey is: .. ' + value2_0)
println('.. Melaleuca ID is: .. ' + value2_1)
println('.. Phone Number is: .. ' + value2_2)

response2 = WS.sendRequest(findTestObject('LINE - XT/1.2 Search Customer', [('Tokenkey') : GlobalVariable.G_TokenKey, ('CustomerID') : GlobalVariable.G_CustomerID]))
println('.. response2 is: .. ' + response2)

def slurper2 = new groovy.json.JsonSlurper()
def result2 = slurper2.parseText(response2.getResponseBodyContent())
def value2 = result2.CustomerID

GlobalVariable.G_CustomerID = value2
println('.. Customer Id is: .. ' + value2)

response3 = WS.sendRequest(findTestObject('LINE - XT/3.1 Insert Contacts', [('TokenKey') : GlobalVariable.G_TokenKey, ('CustomerId') : GlobalVariable.G_CustomerID
            , ('Contact') : GlobalVariable.G3_Contact, ('SpouseContact') : GlobalVariable.G3_SpouseContact]))
println('.. response3 is: .. ' + response3)
