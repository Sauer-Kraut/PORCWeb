import config from "@/config";
import type { PubAccountInfo } from "../models/PubAccountInfo";
import { convertToPubAccountInfoRecv, type PubAccountInfoRecv } from "../models/PubAccountInfoRecv";
import { getClientId } from "./clientIdentification";


export async function postUserInfo(user: PubAccountInfo): Promise<void | string> {
    console.log('Trying to post Account Info');

    const recvStruct: PubAccountInfoRecv = convertToPubAccountInfoRecv(user);
    const requestData = JSON.stringify({
        title: 'Account Info POST Request',
        client_id: getClientId(),
        account_info: recvStruct,
    });

    console.log('Request Data:', requestData);
    try {
        const response = await fetch(`${config.getBackendUrl()}/api/account/setinfo`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: requestData,
        });

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        const jsonData = await response.json();
        // console.log('Success:', data);
        if (jsonData.error != null) {
            console.log('Error occurred: ', jsonData.error);
            return jsonData.error;
        }
    } 
    catch (error) {
        console.log('Error occurred: ', error);
        return String(error);
    }
}