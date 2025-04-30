import { useModal } from 'vue-final-modal';
import ErrorPopupModel from '@/components/modals/ErrorPopupModel.vue';

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

    function handleClose() {
        close();
    }
}
