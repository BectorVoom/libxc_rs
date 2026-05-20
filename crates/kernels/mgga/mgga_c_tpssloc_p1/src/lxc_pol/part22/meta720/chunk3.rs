//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2337/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2337<F: Float>(t1484: F, t4233: F, t5544: F, t828: F, t13222: F, t13228: F, t13350: F, t13351: F, t1510: F, t16944: F, t16949: F, t20969: F, t2618: F, t2643: F, t4178: F, t4255: F, t46577: F, t5585: F, t5591: F, t5611: F, t58550: F, t58569: F, t58574: F, t67568: F, t776: F, t817: F, t819: F, t820: F) -> F {
    let t67783 = t1484 * t4233;
    let t67793 = t5544 * t828;
    let t67826 = F::new(595.0) / F::new(864.0) * t46577 + F::new(5.0) / F::new(128.0) * t4178 * t13350 * t5585 * t4255 + t2643 * t13222 * t1510 * t67783 / F::new(128.0) - F::new(3.0) / F::new(128.0) * t4178 * t13222 * t5585 * t13351 - F::new(35.0) / F::new(72.0) * t58550 - t4178 * t13222 * t13228 * t67793 / F::new(128.0) - t4178 * t13222 * t13228 * t5611 * t776 / F::new(128.0) - F::new(5.0) / F::new(256.0) * t2643 * t13350 * t1510 * t16949 - t4178 * t13222 * t13228 * t67783 / F::new(64.0) - t2618 * t20969 / F::new(3072.0) - t817 * t819 * t820 * t67568 / F::new(3072.0) - F::new(5.0) / F::new(128.0) * t2643 * t13350 * t1510 * t16944 + t2643 * t13222 * t58569 * t5591 / F::new(256.0) + F::new(595.0) / F::new(1152.0) * t58574;
    t67826
}
