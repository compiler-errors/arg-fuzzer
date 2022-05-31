
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2595(_: S1, _: S4, _: S7) {}
        
        fn test2595() { foo2595(S1, S2, S6, S4, S6); }
    