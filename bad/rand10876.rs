
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10876(_: S3, _: S6, _: S8) {}
        
        fn test10876() { foo10876(S1, S3, S4, S5, S6, S7); }
    