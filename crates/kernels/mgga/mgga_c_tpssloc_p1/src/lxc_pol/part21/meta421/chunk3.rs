//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1943/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1943<F: Float>(t14701: F, t14833: F, t14835: F, t14837: F, t14840: F, t14844: F, t14847: F, t14849: F, t14852: F, t14857: F, t14860: F, t14862: F, t14864: F, t14866: F, t14916: F, t14936: F, t14939: F) -> F {
    let t15035 = t14701 - t14833 - t14835 - t14837 - t14840 + t14844 + t14847 + t14849 + t14852 - t14857 - t14860 - t14862 + t14864 + t14866 + t14916 + t14936 + t14939;
    t15035
}
