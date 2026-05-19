//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 717/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk717<F: Float>(t349: F, t6768: F, t1946: F, t225: F, t1065: F, t1955: F, t3174: F, t1949: F, t968: F, t1920: F, t6688: F) -> (F, F, F, F, F, F) {
    let t6769 = t349 * t6768;
    let t6771 = t1946 * t225;
    let t6775 = t1955 * t1065;
    let t6776 = t3174 * t6775;
    let t6781 = t968 * t1949;
    let t6783 = F::cast_from(0.27415567780803773942e-2_f64) * t1920 * t6781;
    let t6784 = t6688 * t225;
    (t6769, t6771, t6776, t6781, t6783, t6784)
}
