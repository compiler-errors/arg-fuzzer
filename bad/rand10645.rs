
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10645(_: S5, _: S7) {}
        
        fn test10645() { foo10645(S3, S8, S6, S2, S1, S5, S4); }
    