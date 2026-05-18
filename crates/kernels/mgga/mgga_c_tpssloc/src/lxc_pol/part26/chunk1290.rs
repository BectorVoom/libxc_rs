//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1290/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1290<F: Float>(t23124: F, t81902: F, t81850: F, t81853: F, t81855: F, t81857: F, t81859: F, t81861: F, t81863: F, t81866: F, t81869: F, t81874: F, t81877: F, t81880: F, t81883: F, t81887: F, t81889: F, t81891: F, t81893: F, t81895: F, t81899: F) -> F {
    let t81903 = t81902 * t23124;
    let t81905 = -t81850 - t81853 - F::new(0.12111826828242117256e-2) * t81855 - F::new(35.0) / F::new(192.0) * t81857 + F::new(0.42391393898847410397e-2) * t81859 - F::new(5.0) / F::new(128.0) * t81861 + t81863 / F::new(128.0) + t81866 / F::new(64.0) - F::new(0.20186378047070195427e-3) * t81869 + F::new(0.10093189023535097714e-3) * t81874 + F::new(0.50465945117675488567e-4) * t81877 + t81880 / F::new(1536.0) - F::new(0.15812662803538319751e-2) * t81883 - F::new(7.0) / F::new(384.0) * t81887 + F::new(7.0) / F::new(768.0) * t81889 + F::new(5.0) / F::new(128.0) * t81891 - t81893 / F::new(512.0) - t81895 / F::new(1536.0) + F::new(0.60559134141210586281e-3) * t81899 + F::new(0.3027956707060529314e-3) * t81903;
    t81905
}
