
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1978(_: S5, _: S8) {}
        
        fn test1978() { foo1978(S2, S5, S4, S4, S1, S1, S1); }
    