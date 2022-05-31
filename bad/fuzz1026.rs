
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1026(_: S2, _: S3) {}
        
        fn test1026() { foo1026(S5, S1, S2, S1, S5); }
    