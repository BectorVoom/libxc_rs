//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1115/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1115<F: Float>(t265: F, t394: F, t7458: F, t8675: F, t1873: F, t8103: F, t652: F, t191: F, t192: F, t8107: F, t2020: F, t7688: F, t8690: F, t33043: F) -> (F, F, F, F, F, F, F) {
    let t395 = t265 < t394;
    let t33733 = t7458 * t8675;
    let t33735 = t8103 * t1873;
    let t33736 = t652 * t33735;
    let t33746 = t8107 * t191 * t192;
    let t33747 = t33746 * t2020;
    let t33748 = t8690 * t7688;
    let t33750 = piecewise3::<F>(t395, F::new(0.0), t33043);
    (t33733, t33735, t33736, t33746, t33747, t33748, t33750)
}
