
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4136(_: S6, _: S4) {}
        
        fn test4136() { foo4136(S8, S8, S3, S6, S7); }
    