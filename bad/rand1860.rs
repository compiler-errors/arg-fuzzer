
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1860(_: S2, _: S5, _: S7) {}
        
        fn test1860() { foo1860(S1, S2, S4, S5, S6, S8); }
    