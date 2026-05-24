//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 620/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk620<F: Float>(t2389: F, t3683: F, t774: F, t3610: F, t801: F, t2142: F, t2144: F, t2147: F, t2170: F, t2173: F, t2381: F, t2384: F, t3615: F, t3618: F, t3622: F, t3626: F, t3632: F, t3635: F, t3638: F, t3667: F, t3671: F, t3678: F, t3681: F, t761: F, t771: F, t797: F) -> (F, F, F) {
    let t3685 = t2389 * t774 * t3683;
    let t3689 = t801 * t774 * t3610;
    let t3692 = t2142 + F::new(7.0) / F::new(144.0) * t2144 + F::new(7.0) / F::new(144.0) * t3615 + t2147 * t3618 / F::new(16.0) - t761 * t3622 / F::new(48.0) + t3626 * t3632 / F::new(1536.0) + F::new(7.0) / F::new(4608.0) * t3635 + t2173 * t3638 / F::new(768.0) - t771 * t3667 / F::new(3072.0) - t2173 * t3671 / F::new(3072.0) + F::new(7.0) / F::new(4608.0) * t2170 + t2381 + F::new(7.0) / F::new(1152.0) * t2384 + t2173 * t3678 / F::new(768.0) + F::new(7.0) / F::new(1152.0) * t3681 + F::new(5.0) / F::new(768.0) * t797 * t3685 - t797 * t3689 / F::new(768.0);
    (t3685, t3689, t3692)
}
