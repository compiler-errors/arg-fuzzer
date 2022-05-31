
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10884(_: S5, _: S5, _: S0, _: S2, _: S2, _: S6) {}
        
        fn test10884() { foo10884(S1, S3, S1, S5, S5, S4, S3); }
    