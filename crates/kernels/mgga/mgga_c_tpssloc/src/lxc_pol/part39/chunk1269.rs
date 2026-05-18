//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1269/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1269<F: Float>(t16153: F, t3870: F, t820: F, t3799: F, t5289: F, t11984: F, t15876: F, t15878: F, t15880: F, t15887: F, t15888: F, t15889: F, t15891: F, t15894: F, t15896: F, t15898: F, t15910: F, t9457: F, t9476: F, t9484: F, t9780: F) -> (F, F, F) {
    let t16155 = t3870 * t820 * t16153;
    let t16159 = F::new(7.0) / F::new(2304.0) * t3799 * t5289;
    let t16160 = -t9457 + t9476 + t9484 + t15876 - t15878 + t15880 - t15887 - t15888 - t15889 - t15891 - t15894 - t15896 - t11984 - t15898 + t9780 + t15910;
    (t16155, t16159, t16160)
}
