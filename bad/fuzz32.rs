
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo32(_: S4, _: S5) {}
        
        fn test32() { foo32(S6, S4, S8, S1, S3, S6, S2); }
    