//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1030/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1030<F: Float>(t113: F, t119824: F, t119826: F, t119830: F, t120669: F, t123844: F, t123947: F, t123975: F, t1307: F, t15868: F, t1983: F, t22574: F, t23938: F, t24432: F, t26977: F, t27180: F, t27219: F, t31304: F, t32186: F, t32194: F, t32212: F, t33790: F, t5161: F, t6876: F, t6879: F, t7042: F, t7685: F, t7806: F, t7904: F, t7939: F, t7941: F, t7943: F, t8804: F) -> F {
    let t123981 = -t119824 - t119826 - t119830 - F::new(2.0) * t31304 * t7943 - t113 * (t123844 + t123947) + F::new(2.0) * t31304 * t7941 - F::new(3.0) * t1983 * t32212 * t120669 - t1983 * t8804 * t15868 - F::new(4.0) * t7042 * t27219 - F::new(4.0) * t23938 * t7806 - F::new(4.0) * t26977 * t7806 - F::new(4.0) * t7042 * t27180 - F::new(6.0) * t22574 * t24432 * t7939 * t1307 - F::new(2.0) * t7685 * t32194 - F::new(3.0) * t6876 * t33790 + F::new(6.0) * t31304 * t7904 + F::new(3.0) * t1983 * t123975 * t6879 - t1983 * t32186 * t5161;
    t123981
}
