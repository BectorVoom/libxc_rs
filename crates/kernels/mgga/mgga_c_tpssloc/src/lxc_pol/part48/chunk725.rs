//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 725/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk725<F: Float>(t24721: F, t24722: F, t3502: F, t3504: F, t3500: F, t7337: F, t1202: F, t7344: F, t483: F, t3068: F, t1244: F, t2132: F, t24683: F, t225: F, t460: F, t479: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t24723 = t24721 * t24722;
    let t24727 = t3502 * sigma2;
    let t24728 = t24727 * t3504;
    let t24729 = t3500 * t24728;
    let t24732 = t7337 * t3504;
    let t24733 = t3500 * t24732;
    let t24736 = t1202 * t7344;
    let t24739 = sigma2 * t483;
    let t24740 = t24739 * t3068;
    let t24741 = t1244 * t24740;
    let t24744 = t2132 * t24683;
    let t24745 = t460 * t225;
    let t24746 = t24745 * t479;
    (t24723, t24729, t24733, t24736, t24741, t24744, t24746)
}
