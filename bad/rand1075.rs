
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1075(_: S6, _: S7) {}
        
        fn test1075() { foo1075(S2, S5, S1, S6, S5); }
    