//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1081/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1081<F: Float>(t32677: F, t652: F, t1874: F, t24999: F, t7685: F, t8490: F, t1842: F, t8485: F, t3887: F, t8475: F, t12021: F, t31090: F) -> (F, F, F, F, F, F) {
    let t32678 = t652 * t32677;
    let t32679 = F::new(2.0) * t32678;
    let t32680 = t24999 * t1874;
    let t32684 = t7685 * t8490;
    let t32685 = t8485 * t1842;
    let t32686 = t3887 * t32685;
    let t32689 = t8475 * t1842;
    let t32690 = t12021 * t32689;
    let t32693 = t31090 * t1842;
    (t32679, t32680, t32684, t32686, t32690, t32693)
}
