
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1699(_: S5, _: S4) {}
        
        fn test1699() { foo1699(S2, S5, S7, S8); }
    