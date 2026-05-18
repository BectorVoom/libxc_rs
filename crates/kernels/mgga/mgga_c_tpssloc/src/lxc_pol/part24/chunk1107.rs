//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1107/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1107<F: Float>(t212: F, t562: F, t6890: F, t22642: F, t225: F, t3879: F, t567: F, t214: F, t1985: F, t1385: F, t6992: F, t3887: F) -> (F, F, F, F, F, F, F) {
    let t22643 = t212 * t562;
    let t22644 = t22643 * t6890;
    let t22645 = t22642 * t22644;
    let t22646 = F::new(0.82246703342411321824e-2) * t22645;
    let t22648 = t3879 * t225 * t567;
    let t22649 = t214 * t22648;
    let t22650 = t1985 * t22649;
    let t22652 = t6992 * t1385;
    let t22653 = t3887 * t22652;
    (t22643, t22644, t22646, t22648, t22649, t22650, t22653)
}
