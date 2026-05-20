//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1663/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1663<F: Float>(t11984: F, t1307: F, t1388: F, t15868: F, t15872: F, t15876: F, t15878: F, t15880: F, t15883: F, t15887: F, t15888: F, t15889: F, t15891: F, t15894: F, t15896: F, t15898: F, t15899: F, t3698: F, t3914: F, t5126: F, t5160: F, t5161: F, t9457: F, t9476: F, t9484: F, t9780: F) -> F {
    let t15903 = F::new(12.0) * t1307 * t15883 * t5126 - F::new(2.0) * t1388 * t15868 * t5160 + F::new(2.0) * t15899 * t3698 * t5160 - t3914 * t5160 * t5161 + F::new(6.0) * t15872 * t5126 - t11984 + t15876 - t15878 + t15880 - t15887 - t15888 - t15889 - t15891 - t15894 - t15896 - t15898 - t9457 + t9476 + t9484 + t9780;
    t15903
}
