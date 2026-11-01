import { useModal } from 'vue-final-modal';
import ErrorPopupModel from '@/vue/global/system/modals/ErrorModal.vue';

export function showErrorModal(errorMessage: string = 'An unexpected error occurred.') {
    const { open, close } = useModal({
        component: ErrorPopupModel,
        attrs: {
            errorMessage,
            onClose: () => {
                console.log('Error modal closed');
                handleClose();
            },
        },
    });
    open();
    //console.log('I was told to show this error here:', errorMessage);

    function handleClose() {
        close();
    }
}
