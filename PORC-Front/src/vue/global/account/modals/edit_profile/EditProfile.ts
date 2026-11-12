import type { AccountCustomisation } from "@/models/pub_account_info/account_cust/AccountCustomisation";
import type { PubAccountInfo } from "@/models/pub_account_info/PubAccountInfo";
import { useModal } from "vue-final-modal";
import EditProfileModal from "./EditProfile.vue";

export function EditProfile(account: PubAccountInfo) {
    const { open, close } = useModal({
        component: EditProfileModal,
        attrs: {
            account: account,
            async onClose() {
                close();
            },
            async onSave(data: AccountCustomisation) {
                close();
                
            },
        },
    });
    open();
}