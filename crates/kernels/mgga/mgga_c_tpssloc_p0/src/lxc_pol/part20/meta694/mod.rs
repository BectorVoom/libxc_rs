//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta694 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2643;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2644;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2645;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta694<F: Float>(t28: F, t3673: F, t584: F, t1081: F, t3231: F, t16: F, t5181: F, t591: F, t11122: F, t12000: F, t12001: F, t1302: F, t16003: F, t16006: F, t1649: F, t2: F, t3711: F, t39877: F, t5178: F, zeta_threshold: F, t53827: F, t16465: F, t225: F, t12344: F, t5234: F, t1369: F, t16336: F, t3876: F, t16333: F, t3866: F, t1831: F, t40284: F, t12339: F, t5314: F, t40059: F, t3872: F, t12336: F, t12361: F, t1363: F, t1367: F, t16321: F, t3783: F, t40287: F, t5240: F, t820: F) -> (F, F, F, F, F, F, F) {
        let (t53832, t53835, t53841, t53844, t53854) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2643::<F>(t28, t3673, t584, t1081, t3231, t16, t5181, t591, t11122, t12000, t12001, t1302, t16003, t16006, t1649, t2, t3711, t39877, t5178, zeta_threshold);
        let t53856 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2644::<F>(t53827, t53854);
        let (t53866, t53882, t53883, t53893, t53895, t53897) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2645::<F>(t16465, t225, t12344, t5234, t1369, t16336, t3876, t16333, t3866, t1831, t40284, t12339, t5314);
        let t53905 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2646::<F>(t1831, t40059, t16336, t3872, t12336, t12361, t1363, t1367, t16321, t16333, t3783, t40287, t5240, t5314, t53856, t53882, t53883, t53893, t53895, t53897, t820);
    (t53832, t53835, t53841, t53844, t53856, t53866, t53905)
}
