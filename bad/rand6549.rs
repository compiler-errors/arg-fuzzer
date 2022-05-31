
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo6549(_: S6, _: S3, _: S3) {}
        
        fn test6549() { foo6549(S2, S4, S5, S6, S8); }
    