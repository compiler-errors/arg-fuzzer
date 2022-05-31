
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2497(_: S2, _: S8, _: S7, _: S5) {}
        
        fn test2497() { foo2497(S6, S4, S4, S1, S2, S6, S2); }
    