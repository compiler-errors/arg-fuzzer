
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10538(_: S4, _: S6, _: S7, _: S8) {}
        
        fn test10538() { foo10538(S1, S2, S3, S4, S5, S7, S8); }
    