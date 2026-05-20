//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2432/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2432<F: Float>(t1041: F, t13969: F, t14188: F, t1046: F, t10898: F, t10949: F, t13977: F, t13982: F, t13987: F, t1618: F, t3043: F, t42595: F, t43120: F, t43322: F, t43343: F, t4596: F, t4652: F, t49721: F, t49732: F, t49734: F, t49740: F, t49743: F) -> F {
    let t49748 = t1041 * t13969 * t14188;
    let t49750 = t49721 / F::new(1536.0) + t43343 * t4596 / F::new(512.0) + t10949 * t13977 / F::new(256.0) + t10949 * t13982 / F::new(512.0) + F::new(3.0) / F::new(512.0) * t43322 * t13987 + t49732 / F::new(48.0) + t49734 / F::new(1536.0) - t10898 * t4652 / F::new(96.0) - t43120 * t1618 / F::new(192.0) - t49740 * t1046 / F::new(144.0) + t49743 * t3043 / F::new(192.0) + F::new(5.0) / F::new(7776.0) * t42595 + F::new(5.0) / F::new(2592.0) * t49748;
    t49750
}
